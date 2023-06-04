use std::{borrow::Cow, time::Instant};

use anyhow::Context;
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

use crate::{shader_list::ShaderList, uniform::Uniform};

pub struct State {
    window: Window,
    pub size: PhysicalSize<u32>,

    shader_list: ShaderList,

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::SurfaceConfiguration,

    time: Instant,

    _sampler: wgpu::Sampler,
    pipeline: wgpu::RenderPipeline,

    uniform: Uniform,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: wgpu::BindGroup,
}

impl State {
    pub async fn new(window: Window, shader_list: ShaderList) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let (adapter, surface, device, queue) = get_surface_device_queue(&window).await?;

        let config = gen_config(adapter, &surface, &size)?;

        surface.configure(&device, &config);

        let (uniform, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            crate::uniform::setup_uniform(&device);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let (shader_name, fragment_shader) = shader_list.current_shader();

        window.set_title(shader_name);

        let pipeline = build_pipeline(
            &device,
            &[&uniform_bind_group_layout],
            config.format,
            fragment_shader,
        );

        Ok(Self {
            window,
            size,

            shader_list,

            surface,
            device,
            queue,

            config,

            time: Instant::now(),

            _sampler: sampler,
            pipeline,

            uniform,
            uniform_bind_group,
            uniform_bind_group_layout,
            uniform_buffer,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    },
                ..
            } => {
                let shader = self.shader_list.previous_shader();
                self.update_shader(shader);
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    },
                ..
            } => {
                let shader = self.shader_list.next_shader();
                self.update_shader(shader);
                true
            }
            _ => false,
        }
    }

    fn update_shader(&mut self, shader: (&str, String)) {
        let (title, content) = shader;

        self.window.set_title(title);
        self.pipeline = build_pipeline(
            &self.device,
            &[&self.uniform_bind_group_layout],
            self.config.format,
            content,
        );
    }

    pub fn update(&mut self) {
        // No-op
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.uniform.resolution = [new_size.width as f32, new_size.height as f32];
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render-encoder"),
            });

        self.uniform.time = self.time.elapsed().as_secs_f32();

        self.queue
            .write_buffer(&self.uniform_buffer, 0, self.uniform.as_ref());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // load: wgpu::LoadOp::Clear(wgpu::Color {
                        //     r: 1.,
                        //     g: 1.,
                        //     b: 1.,
                        //     a: 1.,
                        // }),
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }
}

async fn get_surface_device_queue(
    window: &Window,
) -> anyhow::Result<(wgpu::Adapter, wgpu::Surface, wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface =
        unsafe { instance.create_surface(window) }.context("Failed to create the surface")?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("Can't request a compatible adapter")?;

    let adapter_info = adapter.get_info();

    log::info!(
        "Requested adapter: `{}` with driver `{}`",
        adapter_info.name,
        adapter_info.driver
    );

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("wgpu-device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .context("Can't request compatible device")?;

    Ok((adapter, surface, device, queue))
}

fn gen_config(
    adapter: wgpu::Adapter,
    surface: &wgpu::Surface,
    size: &PhysicalSize<u32>,
) -> anyhow::Result<wgpu::SurfaceConfiguration> {
    let caps = surface.get_capabilities(&adapter);

    let surface_format = caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .context("Can't find a compatible surface format")?;

    Ok(wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: caps.present_modes[0],
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
    })
}

fn build_pipeline(
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    texture_format: wgpu::TextureFormat,
    fragment_shader: String,
) -> wgpu::RenderPipeline {
    let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("vertex-shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::from(include_str!("assets/vertex.wgsl"))),
    });

    let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fragment-shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::from(fragment_shader)),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("render-pipeline-layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}
