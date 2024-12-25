import * as fs from "fs";
import path from "path";

const filePath = path.join(__dirname, "input.txt");
let input: string | null = null;

fs.readFile(filePath, { encoding: "utf8" }, (err, data) => {
	if (!err) {
		input = data;
		console.log(input);
	} else {
		console.log("couldn't read file");
	}
});

let story = 0;

// if (input) {
// 	for (ch of input) {
// 		switch (ch) {
// 			case "(":
// 				story++;
// 				break;
// 			case ")":
// 				story--;
// 				break;
// 			default:
// 				console.log("invalid input");
// 		}
// 	}
// 	console.log(`answer: ${story}`);
// } else {
// 	console.log("cannot perform operations on null");
// }
