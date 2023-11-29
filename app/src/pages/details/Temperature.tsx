import { useState, useEffect } from "react";
import Layout from "../../components/Layout";
import Header from "../../components/Header";
import { SERVER_URL } from "../../constants";
import {
  Calendar,
  Information,
  Normality,
  Title,
} from "../../components/Details";
import axios from "axios";

export default function Temperature() {
  let [select, setSelect] = useState("day");
  let [values, setValues] = useState<number[]>([]);

  useEffect(() => {
    axios
      .post(`${SERVER_URL}/temperature`, { type: select })
      .then((res: any) => {
        if (res.data.payload) {
          console.log(res.data.payload.temperature);
          setValues([res.data.payload.temperature, 20]);
        }
      });
  }, [select]);

  return (
    <Layout>
      <div className="w-screen h-screen p-5 grid grid-rows-[1fr_25fr] grid-cols-3 gap-5">
        <Header />
        <div className="col-span-3 grid grid-rows-[1fr_5fr_5fr_2fr] gap-5">
          <Title value="Temperatura" />
          <Calendar
            set={setSelect}
            unit={select}
            data={values}
            min={15}
            max={30}
            stepSize={5}
          />
          <Information min={15} max={30} average={25} noise={0.5} />
          <Normality value={40} />
        </div>
      </div>
    </Layout>
  );
}
