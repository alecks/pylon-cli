import typescript from "@rollup/plugin-typescript";

export default {
    input: "src/main.ts",
    output: [
        {
            file: "dist/bundle.js",
            format: "umd",
        },
    ],
    plugins: [typescript({ lib: ["es2020"], target: "es2020" })],
};
