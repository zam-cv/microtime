import { useState, useEffect } from "react";
import { faker } from "@faker-js/faker";
import GraphicView from "./GraphicView";
import { CHART_OPTIONS } from "../constants";
import { Link } from "react-router-dom";

const labels = ["1", "2", "3", "4", "5", "6", "7"];
const options = JSON.parse(JSON.stringify(CHART_OPTIONS));
options.plugins.title.text = "Pulsos por Minuto";
options.scales.y.min = 0;
options.scales.y.max = 150;
options.scales.y.ticks.stepSize = 130;

export const heartRate = {
  title: "Pulsos por Minuto",
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: 0, max: 150 })),
      borderColor: "rgb(255, 99, 132)",
    },
  ],
};

export default function HeartRate() {
  let [heartRateValues, setHeartRate] = useState(heartRate);

  useEffect(() => {
    setInterval(() => {
      setHeartRate((prev) => {
        return {
          ...prev,
          datasets: [
            {
              ...prev.datasets[0],
              data: prev.datasets[0].data.map(() =>
                faker.number.int({ min: 60, max: 100 })
              ),
            },
          ],
        };
      });
    }, 200);
  }, []);

  return (
    <div className="bg-blue-950 rounded-lg col-span-3 pt-1 p-3">
      <Link to="heart-rate">
        <GraphicView options={options} values={heartRateValues} />
      </Link>
    </div>
  );
}
