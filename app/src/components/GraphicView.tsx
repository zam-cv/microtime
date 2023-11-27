import { CHART_OPTIONS } from "../constants";
import { Line } from "react-chartjs-2";

export default function GraphicView({
  values,
}: {
  values: {
    labels: string[];
    datasets: {
      data: number[];
      borderColor: string;
    }[];
  };
}) {
  return (
    <div className="bg-blue-950 rounded-lg col-span-3 p-3">
      <div className="h-full">
        <Line data={values} options={CHART_OPTIONS} height={"50%"} />
      </div>
    </div>
  );
}