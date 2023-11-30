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
import { getInfo } from "../../utils/values";
import axios from "axios";

interface Data {
  id: number;
  group: number;
  average: number;
}

export default function Temperature() {
  let [select, setSelect] = useState("day");
  let [min, setMin] = useState(0);
  let [max, setMax] = useState(0);
  let [average, setAverage] = useState(0);
  let [noise, setNoise] = useState(0);
  let [normality, setNormality] = useState(0);
  let [values, setValues] = useState<Data[]>([]);

  useEffect(() => {
    axios
      .post(`${SERVER_URL}/temperature`, { unit: select })
      .then((res: any) => {
        console.log(res.data);
        setValues(res.data);
      });
  }, [select]);

  useEffect(() => {
    axios
      .post(`${SERVER_URL}/temperature`, { unit: "day" })
      .then(({ data }: { data: Data[] }) => {
        console.log(data);
        let { min, max, average, noise, normality } = getInfo(data);

        setNoise(Math.floor(noise));
        setMin(Math.floor(min.average));
        setMax(Math.floor(max.average));
        setAverage(Math.floor(average));
        setNormality(normality);
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
            max={40}
            stepSize={5}
          />
          <Information min={min} max={max} average={average} noise={noise} />
          <Normality value={normality} />
        </div>
      </div>
    </Layout>
  );
}
