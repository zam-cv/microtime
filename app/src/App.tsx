import "./App.css";
import { initChart } from "./utils/chart";
import User from "./components/User";
import Steps from "./components/Steps";
import Temperature from "./components/Temperature";
import HeartRate from "./components/HeartRate";
import Oximetry from "./components/Oximetry";

initChart();

function App() {
  return (
    <div className="w-screen h-screen bg-gradient-to-b from-blue-500 to-purple-500 p-5 grid grid-rows-[1fr_3fr_3fr_4fr_15fr] grid-cols-3 gap-5">
      <User name="Rafael Soto" />
      <HeartRate />
      <Oximetry />
      <Steps steps={10} calories={10} />
      <Temperature />
      <div className="bg-blue-950 rounded-lg col-span-3 text-white"></div>
    </div>
  );
}

export default App;