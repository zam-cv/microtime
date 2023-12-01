import Steps from "../components/Steps";
import User from "../components/User";
import Temperature from "../components/Temperature";
import HeartRate from "../components/HeartRate";
import Oximetry from "../components/Oximetry";
import Report from "../components/Report";
import Layout from "../components/Layout";

export default function Home() {
  return (
    <Layout>
      <div className="w-screen h-screen p-5 grid grid-rows-[1fr_3fr_3fr_4fr_15fr] grid-cols-3 gap-5">
        <User name="Rafael Soto" />
        <HeartRate />
        <Oximetry />
        <Steps />
        <Temperature />
        <Report />
      </div>
    </Layout>
  );
}
