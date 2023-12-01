import { useState, useEffect } from "react";
import { faker } from "@faker-js/faker";
import GraphicView from "./GraphicView";
import { CHART_OPTIONS } from "../constants";
import { Link } from "react-router-dom";

const labels = ["1", "2", "3", "4", "5", "6", "7"];
const options = JSON.parse(JSON.stringify(CHART_OPTIONS));
options.plugins.title.text = "Oximetria";
options.scales.y.min = 80;
options.scales.y.max = 100;
options.scales.y.ticks.stepSize = 20;

export const oximetry = {
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: 80, max: 100 })),
      borderColor: "rgb(0, 200, 255)",
    },
  ],
};

export default function Oximetry() {
  let [oximetryValues, setOximetry] = useState(oximetry);

  useEffect(() => {
    setInterval(() => {
      setOximetry((prev) => {
        return {
          ...prev,
          datasets: [
            {
              ...prev.datasets[0],
              data: prev.datasets[0].data.map(() =>
                faker.number.int({ min: 90, max: 93 })
              ),
            },
          ],
        };
      });
    }, 200);
  }, []);

  return (
    <div className="bg-blue-950 rounded-lg col-span-3 pt-1 p-3">
      <Link to="/oximetry">
        <GraphicView options={options} values={oximetryValues} />
      </Link>
    </div>
  );
}
