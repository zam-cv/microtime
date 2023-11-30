import { useState, useEffect } from "react";
import GraphicView from "./GraphicView";
import { CHART_OPTIONS } from "../constants";
import moment from "moment/moment";
import "moment/locale/es";

const UNIT: any = {
  day: ["06:00", "12:00", "18:00", "24:00"],
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
  status,
}: {
  children: React.ReactNode;
  onClick?: () => void;
  status: boolean;
}) {
  return (
    <span
      onClick={onClick}
      className={`border-r border-slate-300 p-2 cursor-pointer ${
        status ? "font-bold text-cyan-400" : ""
      }`}
    >
      {children}
    </span>
  );
}

interface Data {
  id: number;
  group: number;
  average: number;
}

export function Calendar({
  set,
  data,
  unit,
  min,
  max,
  stepSize,
}: {
  set: (type: string) => void;
  data: Data[];
  unit: string;
  min: number;
  max: number;
  stepSize: number;
}) {
  options.scales.y.min = min;
  options.scales.y.max = max;
  options.scales.y.ticks.stepSize = stepSize;

  const [values, setValues] = useState({
    labels: UNIT[unit],
    datasets: [
      {
        data: [0],
        borderColor: "rgb(0, 200, 255)",
      },
    ],
  });

  useEffect(() => {
    setValues({
      labels: data.map((item) => {
        switch (unit) {
          case "day":
            return moment.unix(item.group).format("HH:mm");
          case "week":
            return moment.unix(item.group).format("ddd");
          case "month":
            return moment.unix(item.group).format("MMM DD");
          case "year":
            return moment.unix(item.group).format("YYYY MMM");
          default:
            return "";
        }
      }),
      datasets: [
        {
          data: data.map((item) => item.average),
          borderColor: "rgb(0, 200, 255)",
        },
      ],
    });
  }, [data]);

  function handle(type: string) {
    set(type);
  }

  return (
    <div className="bg-blue-950 rounded-lg p-5 grid grid-rows-[2fr_6fr]">
      <div className="grid grid-cols-4 text-white text-sm items-center text-center pb-3">
        <Button onClick={() => handle("day")} status={unit == "day"}>
          D
        </Button>
        <Button onClick={() => handle("week")} status={unit == "week"}>
          S
        </Button>
        <Button onClick={() => handle("month")} status={unit == "month"}>
          M
        </Button>
        <span
          onClick={() => handle("year")}
          className={`cursor-pointer p-2 ${
            unit == "year" ? "font-bold text-cyan-400" : ""
          }`}
        >
          A
        </span>
      </div>
      <div>
        <GraphicView options={options} values={values} />
      </div>
    </div>
  );
}

export function Card({
  value,
  description,
}: {
  value: number;
  description: string;
}) {
  return (
    <div className="flex items-center justify-center flex-col gap-1">
      <p className="font-bold">{value}</p>
      <p className="text-sm">{description}</p>
    </div>
  );
}

export function Information({
  min,
  max,
  average,
  noise,
}: {
  min: number;
  max: number;
  average: number;
  noise: number;
}) {
  let [date, setDate] = useState("");

  useEffect(() => {
    moment.locale("es");
    var localLocale = moment();
    localLocale.locale(false);
    var date = localLocale.format("DD [de] MMMM [del] YYYY");
    var list = date.split(" ");
    list[2] = list[2].charAt(0).toUpperCase() + list[2].slice(1);
    date = list.join(" ");
    setDate(date);
  }, []);

  return (
    <div className="bg-blue-950 rounded-lg text-white p-5 grid grid-rows-[1fr_4fr]">
      <div className="flex items-center justify-center">
        <p className="text-base font-medium">{date}</p>
      </div>
      <div className="grid grid-rows-2 grid-cols-2 p-3">
        <Card value={max} description="Maximo" />
        <Card value={min} description="Minimo" />
        <Card value={average} description="Promedio" />
        <Card value={noise} description="Ruido" />
      </div>
    </div>
  );
}

export function Normality({ value }: { value: number }) {
  return (
    <div className="bg-blue-950 rounded-lg text-white grid grid-rows-[auto_3fr] p-2">
      <div className="flex items-center justify-center p-3">
        <p className="text-base font-medium">Normalidad</p>
      </div>
      <div className="flex items-center pl-3 pr-3 pb-3">
        <div className="relative w-full flex items-center">
          <div className="w-full h-1 bg-emerald-500 absolute"></div>
          <div className="w-1 h-3 bg-emerald-500 absolute left-1/2"></div>
          <div className="w-1 h-3 bg-emerald-500 absolute left-0"></div>
          <div className="w-1 h-3 bg-emerald-500 absolute right-0"></div>
          <div
            className={`w-1 h-3 bg-red-500 absolute`}
            style={{ left: `${value}%` }}
          ></div>
        </div>
      </div>
    </div>
  );
}

export function Title({ value }: { value: string }) {
  return (
    <div className="bg-blue-950 rounded-lg text-white flex items-center justify-center font-bold">
      {value}
    </div>
  );
}
