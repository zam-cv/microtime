import Layout from "../../components/Layout";
import Header from "../../components/Header";

export default function Oximetry() {
  return (
    <Layout>
      <div className="w-screen h-screen p-5 grid grid-rows-[1fr_25fr] grid-cols-3 gap-5">
        <Header />
        <div className="bg-blue-950 col-span-3 rounded-lg"></div>
      </div>
    </Layout>
  );
}
