import { useState, useEffect } from "react";
import { faker } from "@faker-js/faker";
import GraphicView from "./GraphicView";

const labels = ["1", "2", "3", "4", "5", "6", "7"];

export const oximetry = {
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: -220, max: 220 })),
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
                faker.number.int({ min: -220, max: 220 })
              ),
            },
          ],
        };
      });
    }, 200);
  }, []);

  return <GraphicView values={oximetryValues} />;
}