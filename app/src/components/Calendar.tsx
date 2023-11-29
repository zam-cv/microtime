import { useState, useEffect } from "react";
import GraphicView from "../components/GraphicView";
import { CHART_OPTIONS } from "../constants";

const UNIT: any = {
  day: ["00:00", "06:00", "12:00", "18:00", "24:00"],
  week: ["L", "M", "M", "J", "V", "S", "D"],
  month: ["S1", "S2", "S3", "S4"],
  year: ["E1", "E2", "E3", "E4"],
};

const options = JSON.parse(JSON.stringify(CHART_OPTIONS));
options.plugins.title.display = false;
options.scales.x.display = true;
options.scales.y.ticks.font.size = 11;
options.scales.x.ticks.font.size = 12;

export function Button({
  children,
  onClick,
}: {
  children: React.ReactNode;
  onClick?: () => void;
}) {
  return (
    <span
      onClick={onClick}
      className="border-r border-slate-300 p-2 cursor-pointer"
    >
      {children}
    </span>
  );
}

export default function Calendar({
  set,
  data,
}: {
  set: (type: string) => void;
  data: number[];
}) {
  const [unit, setUnit] = useState(UNIT.day);
  const [values, setValues] = useState({
    labels: UNIT.day,
    datasets: [
      {
        data: [0],
        borderColor: "rgb(0, 200, 255)",
      },
    ],
  });

  useEffect(() => {
    setValues({
      labels: UNIT[unit],
      datasets: [
        {
          data,
          borderColor: "rgb(0, 200, 255)",
        },
      ],
    });
  }, []);

  function handle(type: string) {
    set(type);
    setUnit(UNIT[type]);
  }

  return (
    <div className="bg-blue-950 rounded-lg p-5 grid grid-rows-[2fr_6fr]">
      <div className="grid grid-cols-4 text-white text-sm items-center text-center pb-3">
        <Button onClick={() => handle("day")}>D</Button>
        <Button onClick={() => handle("week")}>S</Button>
        <Button onClick={() => handle("month")}>M</Button>
        <span onClick={() => handle("year")} className="cursor-pointer p-2">
          A
        </span>
      </div>
      <div>
        <GraphicView options={options} values={values} />
      </div>
    </div>
  );
}
