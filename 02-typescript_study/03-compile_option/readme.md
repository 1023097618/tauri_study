# TS编译选项
使用以下命令在检测到`.ts`文件发生变化时自动重新编译到`.js`文件，加上一个`-w`参数
``` cmd
tsc hello.ts -w
```

在和要编译的ts文件同级的目录下创建`tsconfig.json`来对编译进行配置，创建完之后往里面加一个`{}`

创建完之后，使用以下命令对这个项目的ts文件进行编译
``` cmd
tsc -w
```

`tsconfig.json`中支持有以下的值
1. `"include":["./src/**/*"]`这表示src目录下的任意目录下的任意文件
2. `"exclude"`这个默认值是`node_modules，bower_components，jspm_packages和<outDir>`，一般情况下不需要配置
3. `compilerOptions`这个是重点，它下方可以写如下的一些编译选项
    1. `"target"`，用于指定编译后js的版本号，可以选的字符串值为`ES5`、`ES6`，我们创建一个ts文件如下
    ``` ts
    let a=1;
    ```
    - 如果为`ES5`，编译后的效果如下
    ``` js
    var a = 1;
    ```
    - 如果是`ES6`，编译后的效果如下
    ``` js
    let a = 1;
    ```
    2. `"module"`，用于指定模块化的解决方案，可选的值为`ES6`,`commonjs`,我们创建一个ts文件如下
    ``` ts
    import {hi} from './aaa.js';
    console.log(hi);
    ```
    - 如果是`ES6`，编译后的js文件如下
    ``` js
    import { hi } from './aaa.js';
    console.log(hi);
    ```
    - 如果是`commonjs`，编译后的js文件如下
    ``` js
    "use strict";
    Object.defineProperty(exports, "__esModule", { value: true });
    const aaa_js_1 = require("./aaa.js");
    console.log(aaa_js_1.hi);
    ```
    3. `"lib"`后面跟一个数组，指明所有该项目中会用到的库，一般情况下不设置这个值保持默认
    4. `"outDir"`标明编译后的js文件放在哪个目录下，一般写`"outDir":./dist`
    5. `"outFile"`设置之后将所有的js文件合并成同一个，一般不用设置，交给打包工具去做。
    6. `"allowJs"`是否同时编译js代码进来，默认情况为false
    7. `"checkJs"` 是否对js代码也开启ts的语法检查，默认为false。
    比如以下js代码，开启语法检查之后就通不过编译
    ``` js
    let a="111";
    a=111;
    ```
    8. `"removeComments"` 编译后的js代码是否移除注释，默认为false
    9. `"noEmit"` 是否生成编译后的文件，默认为false，如果你只想要用它的语法检查而不想要它编译后的文件可以设为true
    10. `"noEmitOnError"` 是否在出错后不生成编译后的文件，默认为false
    11. `"alwaysStrict"` 是否开启严格模式，开启之后编译后的每个文件开头都会有`"use strict";`，一般情况下建议开启`"alwaysStrict":true`，因为严格模式的代码在浏览器中的运行效率会高一些
    12. `"noImplicitAny"` 是否不允许参数的any类型，默认不开启，开启之后这个ts代码就通不过编译了，因为要给参数声明明确的类型。
    ``` ts
    function add(a,b){
        return a+b;
    }
    ```
    13. `"noImplicitThis"` 是否允许没有类型的this类型，默认不开启，开启之后这个ts代码就不通过编译了，因为这个this指向的值不明确。如果是`函数调用`的话指向的就是`window`(非严格模式)或者`undefined`(严格模式)。如果是`方法调用`的话指向的就是调用它的`object`。
    ``` ts
    function hello(){
        console.log(this)
    }
    ```
    14. `"strictNullChecks"`是否检查空值，开启之后下面代码通不过编译
    ``` ts
    const box1=document.querySelector(".div");
        box1.addEventListener("click",()=>{
    })
    ```
    15. `"strict"` 是否开启严格模式，开启之后，上方对应的`11、12、13、14`条均会开启，是个总开关
