type RawList = &'static [(&'static str, &'static str)];

#[derive(Debug, Clone, Copy)]
pub struct ShaderList {
    prefix: &'static str,
    suffix: &'static str,
    shaders: RawList,
    current_shader: usize,
}

impl ShaderList {
    pub const fn new(prefix: &'static str, suffix: &'static str, shaders: RawList) -> Self {
        assert!(shaders.len() > 0);

        Self {
            prefix,
            suffix,
            shaders,
            current_shader: 0,
        }
    }

    pub fn next_shader(&mut self) -> (&'static str, String) {
        self.increment_index();
        self.current_shader()
    }

    pub fn previous_shader(&mut self) -> (&'static str, String) {
        self.decrement_index();
        self.current_shader()
    }

    pub fn current_shader(&self) -> (&'static str, String) {
        let (shader_name, shader_body) = self.shaders[self.current_shader];

        let full_shader = [self.prefix, shader_body, self.suffix].join("");
        (shader_name, full_shader)
    }

    fn increment_index(&mut self) -> usize {
        self.current_shader = (self.current_shader + 1) % self.shaders.len();
        self.current_shader
    }

    fn decrement_index(&mut self) -> usize {
        self.current_shader = self.current_shader.wrapping_sub(1) % self.shaders.len();
        self.current_shader
    }
}

#[cfg(test)]
mod tests {
    use super::ShaderList;

    const SHADER: ShaderList =
        ShaderList::new("p", "s", &[("1", "a"), ("2", "b"), ("3", "c"), ("4", "d")]);

    #[test]
    fn increment_counter() {
        let mut list = SHADER;

        assert_eq!(list.increment_index(), 1);
        assert_eq!(list.increment_index(), 2);
        assert_eq!(list.increment_index(), 3);
        assert_eq!(list.increment_index(), 0);
    }

    #[test]
    fn decrement_counter() {
        let mut list = SHADER;

        assert_eq!(list.decrement_index(), 3);
        assert_eq!(list.decrement_index(), 2);
        assert_eq!(list.decrement_index(), 1);
        assert_eq!(list.decrement_index(), 0);
    }

    #[test]
    fn shader() {
        let mut list = SHADER;

        macro_rules! compare_shader {
            ($get:expr, $filename:literal, $content:literal) => {{
                let (filename, content) = $get;

                assert_eq!(filename, $filename);
                assert_eq!(content, $content);
            }};
        }

        compare_shader!(list.current_shader(), "1", "pas");
        compare_shader!(list.next_shader(), "2", "pbs");
        compare_shader!(list.next_shader(), "3", "pcs");
        compare_shader!(list.previous_shader(), "2", "pbs");
        compare_shader!(list.previous_shader(), "1", "pas");
        compare_shader!(list.previous_shader(), "4", "pds");
    }
}
