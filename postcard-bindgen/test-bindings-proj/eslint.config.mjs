import js from "@eslint/js";
import babelParser from "@babel/eslint-parser";
import globals from "globals";

export default [
    {
        ignores: ["lib/**"],
    },
    {
        files: ["src/**/*.ts"],
        languageOptions: {
            globals: globals.node,
            parser: babelParser,
            parserOptions: {
                babelOptions: {
                    babelrc: false,
                    configFile: false,
                    presets: ["@babel/preset-typescript"],
                },
                requireConfigFile: false,
            },
        },
        rules: {
            ...js.configs.recommended.rules,
            "no-unused-vars": "off",
        },
    },
];
