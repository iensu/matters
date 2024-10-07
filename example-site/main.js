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

let NUM_PROBLEMS = 30;
let MAX_RESULT = 20;
let MAX_FACTOR = 10;
let OPERATIONS = FLAG_ADD + FLAG_SUB + FLAG_MUL;

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

    results.push({ op, x, y });

    index += PROBLEM_OBJECT_SIZE;
  }

  return results;
};

const render = (renderFn, ...args) => {
  const template = document.createElement("template");

  template.innerHTML = renderFn(...args);

  if (template.content.childNodes.length > 1) {
    return [...template.content.childNodes];
  }

  return template.content.firstChild;
};

const renderProblem = (label, problem) => {
  return `
    <div class="problem">
      <label>${label}</label>
      <span class="number">${problem.x}</span>
      <span class="operation">${OPERATION_STRINGS[problem.op]}</span>
      <span class="number">${problem.y}</span>
      <span class="equals">=</span>
      <input type=""text" class="answer-input" inputmode="numeric" />
    </div>
`;
};

const renderProblems = (problems) =>
  problems.map((problem, index) => renderProblem(index + 1, problem)).join("");

const onSubmit = (problems) => (event) => {
  event.preventDefault();

  const answers = document.querySelectorAll(".answer-input");

  const evaluate = (problem) => {
    switch (problem.op) {
      case FLAG_ADD:
        return problem.x + problem.y;
      case FLAG_SUB:
        return problem.x - problem.y;
      case FLAG_MUL:
        return problem.x * problem.y;
      case FLAG_DIV:
        return problem.x / problem.y;
    }
  };

  answers.forEach((answerElem, index) => {
    answerElem.classList.remove("correct");
    answerElem.classList.remove("incorrect");
    const answer = Number(answerElem.value);
    const problem = problems[index];

    if (!problem) {
      console.error("No problem for answer!", index);
    }
    const result = evaluate(problem) === answer ? "correct" : "incorrect";
    answerElem.classList.add(result);
  });
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

  const form = document.getElementById("problems");

  form.addEventListener("submit", onSubmit(problems));

  for (const node of render(renderProblems, problems)) {
    node.addEventListener("keydown", (event) => {
      event.target.classList.remove("correct");
      event.target.classList.remove("incorrect");
    });
    form.appendChild(node);
  }

  const correctButton = render(
    () => `<button id="correct-button" type="submit">Rätta</button>`,
  );
  correctButton.addEventListener("click", onSubmit(problems));

  form.appendChild(correctButton);
})();
