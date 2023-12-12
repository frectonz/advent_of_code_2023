function main() {
  const input = Deno.readTextFileSync("input.txt");

  const histories = input.trim().split("\n").map((line) =>
    line.trim().split(" ").map((x) => parseInt(x.trim()))
  );

  const extrapolations = [];

  for (const history of histories) {
    const reductions = [];
    for (
      let curr = history;
      !curr.every((v) => v === 0);
      curr = getReduction(curr)
    ) {
      reductions.push(curr);
    }

    reductions.reverse();
    reductions.forEach((reduction, i, arr) => {
      if (i === 0) {
        reduction.push(reduction[0]);
      } else {
        const prev = arr[i - 1];

        const prevLast = prev.at(-1)!;
        const currLast = reduction.at(-1)!;

        reduction.push(currLast + prevLast);
      }
    });

    const extrapolation = reductions.at(-1)!.at(-1)!;
    extrapolations.push(extrapolation);
  }

  const sum = extrapolations.reduce((acc, curr) => acc + curr, 0);
  console.log("Answer", sum);
}

function getReduction(arr: number[]) {
  const reduction = [];

  for (let i = 0; i < arr.length; i++) {
    const curr = arr[i];
    const next = arr.at(i + 1);

    if (next !== undefined) {
      reduction.push(next - curr);
    }
  }

  return reduction;
}

if (import.meta.main) {
  main();
}
