import "./App.css";
import { initChart } from "./utils/chart";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import HeartRate from "./pages/calendars/HeartRate";
import Oximetry from "./pages/calendars/Oximetry";
import Steps from "./pages/calendars/Steps";
import Temperature from "./pages/calendars/Temperature";

initChart();

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/">
          <Route index element={<Home />} />
          <Route path="heart-rate" element={<HeartRate />} />
          <Route path="oximetry" element={<Oximetry />} />
          <Route path="steps" element={<Steps />} />
          <Route path="temperature" element={<Temperature />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
