pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;
    uniform mat4 uTransform;

    varying lowp vec4 vColor;

    void main() {
        vColor = aColor;
        gl_Position = uTransform * aPosition;
    }
"#;