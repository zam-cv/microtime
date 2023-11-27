import { useState, useEffect } from "react";
import { faker } from "@faker-js/faker";
import GraphicView from "./GraphicView";

const labels = ["1", "2", "3", "4", "5", "6", "7"];

export const heartRate = {
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: -220, max: 220 })),
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
                faker.number.int({ min: -220, max: 220 })
              ),
            },
          ],
        };
      });
    }, 200);
  }, []);

  return <GraphicView values={heartRateValues} />
}