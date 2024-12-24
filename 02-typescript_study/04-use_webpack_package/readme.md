# 使用webpack打包ts代码
创建新文件夹，进入目录，输入以下命令来创建一个新的文件
``` cmd
npm init -y
```
输入以下命令装相关的依赖包
``` cmd
npm i -D webpack webpack-cli typescript ts-loader
```
- `webpack`是打包工具
- `webpack-cli`让这个打包工具可以通过命令行调用
- `typescript`编译typescript代码要用
- `ts-loader`让ts和webpack可以一起用

1. 在项目根目录创建`webpack.config.js`来对webpack进行配置

    初始时期是这样的
    ``` js
    module.exports={
    }
    ```

    使用以下语法来导入一个模块
    ``` js
    const path= require("path");
    ```

    在`webpack.config.js`中的exports中可以写一些属性，比如
    1. `entry:'./src/index.ts'`，指明程序的入口文件
    2. `output`，指明程序的输出文件，一般是这么配置
        ``` js
        const path= require("path");
        module.exports={
            entry:'./src/index.ts',
            output:{
                //这边一定要绝对路径，所以只能这么写
                path:path.resolve(__dirname,"dist"),
                filename:"bundle.js"
            }
        }
        ```
    
    3. `modules`标明使用的模块，一般是这么配置的
        ``` js
            module:{
                rules:[
                    {
                        //匹配对应的文件应用对应的模块
                        test:/\.ts$/,
                        //对应的模块
                        use:'ts-loader',
                        //不匹配的文件
                        exclude: /node_modules/
                    }
                ]
            }
        ```

2. 在项目根目录中创建`tsconfig.json`，在其中写入
    ``` json
    {
        "compilerOptions": {
            "target": "ES6",
            "module": "ES6",
            "strict": true
        }
    }
    ```

3. 在`package.json`中的`scripts`下加入
    ``` json
    "build":"webpack"
    ```

4. 使用以下代码来对现有项目进行打包
    ``` cmd
    npm run build
    ```

5. 使用`html-webpack-plugin`来对有含有html的项目进行打包，使用以下命令对它进行安装
    ``` cmd
    npm i -D html-webpack-plugin
    ```
6. 引入该依赖
    ``` js
    const htmlWebpackPlugin=require("html-webpack-plugin");
    //module.export=...
        plugins:[
            new htmlWebpackPlugin()
        ]
    ```
    >在其中可以通过`template`字段来来输入自定义模板的路径，是这样的`        new htmlWebpackPlugin({template:"./src/index.html"})`

7. 安装`webpack-dev-server`
    
    这个包可以帮助我们检测到代码更改的时候自动重新打包文件，使用以下命令对他进行安装
    ``` cmd
    npm i -D webpack-dev-server
    ```
    继续到`package.json`中的`script`中这么配置
    ``` json
    "start": "webpack server --mode development"
    ```
    然后输入命令
    ``` cmd
    npm run start
    ```

8. 安装`clean-webpack-plugin`
    
    这个包可以帮助我们在热更新重新编译的时候清空输出目录下的文件

    使用以下命令进行安装
    ``` cmd
    npm i -D clean-webpack-plugin
    ```
    ``` js
    const {cleanWebpackPlugin}=require("clean-webpack-plugin");
    //module.export....
    plugins:[
        new htmlWebpackPlugin({template:"./src/index.html"}),
        new cleanWebpackPlugin()
    ]
    ```

9. 支持导入ts模块

    默认情况下，webpack不知道可以导入ts文件，比如你`import a from 'xxx'`，它不会自动帮你补全后面的`xxx.ts`，这时候需要配置一下
    ``` js
    resolve:{
        extensions:['.ts','.js']
    }
    ```

10. 兼容性支持

    如果想要支持一下比较老的浏览器，就需要使用`babel`，它可以让我们的代码兼容老的版本，使用如下命令进行安装
    ``` cmd
    npm -D i @babel/core @babel/preset-env babel-loader core-js
    ```
    使用以下代码进行配置
    ``` js
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
                                        //指定corejs版本，去package.json中看，能够对一些Promise做兼容
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
    }
    ```
    在`enviroument`中再配置以下信息禁用箭头函数以支持IE
    ``` js
        environment:{
            arrowFunction:false
        }
    ```