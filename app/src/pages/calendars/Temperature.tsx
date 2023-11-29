import { useState, useEffect } from "react";
import Layout from "../../components/Layout";
import Header from "../../components/Header";
import { SERVER_URL } from "../../constants";
import Calendar from "../../components/Calendar";
import axios from "axios";

export default function Temperature() {
  let [select, setSelect] = useState("day");
  let [values, setValues] = useState([]);

  useEffect(() => {
    axios.post(`${SERVER_URL}/temperature`, { type: select }).then((res) => {
      setValues(res.data);
    });
  }, [select]);

  return (
    <Layout>
      <div className="w-screen h-screen p-5 grid grid-rows-[1fr_25fr] grid-cols-3 gap-5">
        <Header />
        <div className="col-span-3 grid grid-rows-[1fr_5fr_5fr_2fr] gap-5">
          <div className="bg-blue-950 rounded-lg text-white flex items-center justify-center font-bold">
            Temperatura
          </div>
          <Calendar set={setSelect} data={values} />
          <div className="bg-blue-950 rounded-lg text-white"></div>
          <div className="bg-blue-950 rounded-lg"></div>
        </div>
      </div>
    </Layout>
  );
}
