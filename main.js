const PROBLEM_OBJECT_SIZE = 3; // Three u32 integers

const FLAG_ADD = 0b0001;
const FLAG_SUB = 0b0010;
const FLAG_MUL = 0b0100;
const FLAG_DIV = 0b1000;
const OPERATION_STRINGS = {
  [FLAG_ADD]: "+",
  [FLAG_SUB]: "-",
  [FLAG_MUL]: "*",
  [FLAG_DIV]: "/",
};

let NUM_PROBLEMS = 20;
let MAX_RESULT = 20;
let MAX_FACTOR = 10;
let OPERATIONS = FLAG_ADD + FLAG_SUB;

const createInstance = async (path, importObject = {}) => {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch(path),
    importObject,
  );

  return instance;
};

const generateMathProblems = () => {
  const memory = MattersInstance.exports.memory;
  const pointer = MattersInstance.exports.generate_problems(
    NUM_PROBLEMS,
    MAX_RESULT,
    MAX_FACTOR,
    OPERATIONS,
    0,
  );

  const buffer = memory.buffer;
  const view = new Uint32Array(
    memory.buffer,
    pointer,
    NUM_PROBLEMS * PROBLEM_OBJECT_SIZE,
  );

  const results = [];
  let index = 0;
  while (index < view.length) {
    const [op, x, y] = view.slice(index, index + PROBLEM_OBJECT_SIZE);

    results.push({ op: OPERATION_STRINGS[op], x, y });

    index += PROBLEM_OBJECT_SIZE;
  }

  return results;
};

(async () => {
  const instance = await createInstance("matters.wasm", {
    logger: {
      info(x) {},
      error(code) {
        console.error("Error code:", code);
      },
    },
    random: {
      seed() {
        return BigInt(Math.round(Math.random() * Number.MAX_SAFE_INTEGER));
      },
    },
  });

  window.MattersInstance = instance;

  const problems = generateMathProblems();

  const list = document.getElementById("problems");

  for (const problem of problems) {
    const problemString = `${problem.x} ${problem.op} ${problem.y} = _______`;
    const element = document.createElement("li");
    element.innerText = problemString;
    list.appendChild(element);
  }
})();
