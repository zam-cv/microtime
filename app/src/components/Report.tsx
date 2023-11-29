export function Header() {
  return (
    <div className="bg-blue-900 grid-cols-[3fr_7fr] rounded-lg text-white font-medium grid p-3 gap-3">
      <p className="flex items-center justify-center">Fecha</p>
      <p className="flex items-center justify-center border-white border-l pr-3">
        Descripción
      </p>
    </div>
  );
}

export function Item({
  date,
  time,
  description,
}: {
  date: string;
  time: string;
  description: string;
}) {
  return (
    <div className="bg-blue-900 grid grid-cols-[3fr_7fr_auto] rounded-lg p-3 gap-3 text-white">
      <div className="text-sm">
        <div className="font-bold">{date}</div> <div>{time}</div>
      </div>
      <div className="pl-7 text-sm flex items-center">{description}</div>
      <div className="flex items-center justify-center p-2">
        <div className="rounded-xl h-2 w-2 bg-emerald-500"></div>
      </div>
    </div>
  );
}

export default function Report() {
  return (
    <div className="bg-blue-950 rounded-lg col-span-3 p-5 grid grid-cols-1 grid-rows-[auto_1fr_6fr] gap-3 h-full overflow-auto">
      <p className="text-white text-center font-bold">Reporte</p>
      <Header />
      <div className="overflow-auto grid-cols-3 flex flex-col gap-3 h-full relative">
        <Item date="28/Nov/23" time="04:32" description="Frecuencia elevada" />
        <Item date="27/Nov/23" time="02:12" description="Frecuencia elevada" />
        <Item date="22/Nov/23" time="07:10" description="Frecuencia elevada" />
        <Item date="21/Nov/23" time="01:32" description="Frecuencia elevada" />
        <Item date="20/Nov/23" time="04:32" description="Frecuencia elevada" />
        <Item date="19/Nov/23" time="02:12" description="Frecuencia elevada" />
        <Item date="18/Nov/23" time="07:10" description="Frecuencia elevada" />
        <Item date="17/Nov/23" time="01:32" description="Frecuencia elevada" />
        <Item date="16/Nov/23" time="04:32" description="Frecuencia elevada" />
      </div>
    </div>
  );
}
