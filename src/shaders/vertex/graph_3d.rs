pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute float aY;
    attribute vec3 aVertexNormal;

    uniform mat4 uNormalsRotation;
    uniform mat4 uProjection;
    varying lowp vec4 vColor;

    void main() {
        gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);

        vec3 ambientLight = vec3(0.2, 0.2, 0.2);
        vec3 directionalLightColor = vec3(1, 1, 1);
        vec3 directionalVector = normalize(vec3(-0.85, 0.8, 0.75));

        vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
        float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
        vec3 vLighting = ambientLight + (directionalLightColor * directional);
        vec3 baseColor = vec3(0.5, 0.5, 0.8);

        vColor = vec4(baseColor * vLighting, 1.0);
    }
"#;