var Gender;
(function (Gender) {
    Gender[Gender["Female"] = 0] = "Female";
    Gender[Gender["Male"] = 1] = "Male";
})(Gender || (Gender = {}));
;
var person;
person = {
    name: "小明",
    gender: Gender.Male
};
