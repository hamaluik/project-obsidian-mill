precision mediump float;

uniform sampler2D IMAGE;

varying vec2 v_uv;

void main() {
	gl_FragColor = texture2D(IMAGE, v_uv);
}