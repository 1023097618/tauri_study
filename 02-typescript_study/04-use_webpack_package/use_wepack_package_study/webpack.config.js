const path = require("path");
const htmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
module.exports = {
    entry: './src/index.ts',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
        environment:{
            arrowFunction:false
        }
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                //从后往前use，先用ts-loader转换为js，再用babel-loader把js转换为兼容的
                use: [
                    {
                        loader: "babel-loader",
                        options: {
                            presets: [
                                [
                                    //指定环境的插件
                                    "@babel/preset-env",
                                    {
                                        //要兼容的目标浏览器
                                        targets: {
                                            "chrome": "88",
                                            "ie":"11"
                                        },
                                        //指定corejs版本，去package.json中看
                                        "corejs": "3",
                                        //使用corejs的方式，按需加载
                                        "useBuiltIns": "usage"
                                    }
                                ]
                            ]
                        }
                    },
                    'ts-loader'],
                exclude: /node_modules/
            }
        ]
    },
    plugins: [
        new htmlWebpackPlugin({ template: "./src/index.html" }),
        new CleanWebpackPlugin()
    ],
    resolve: {
        extensions: ['.ts', '.js']
    }
}