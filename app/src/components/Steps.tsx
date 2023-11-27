export default function Steps({
  steps,
  calories,
}: {
  steps: number;
  calories: number;
}) {
  return (
    <div className="bg-blue-950 rounded-lg col-span-2 p-5 flex justify-center items-center">
      <div className="grid grid-rows-1 grid-cols-2 gap-5 w-full">
        <div className="flex justify-center items-center flex-col">
          <div className="flex gap-2 mb-1">
            <svg viewBox="0 0 512 512" fill="currentColor" className="w-5 h-5">
              <path
                fill="none"
                stroke="white"
                strokeMiterlimit={10}
                strokeWidth={20}
                d="M200 246.84c8.81 58.62-7.33 90.67-52.91 97.41-50.65 7.49-71.52-26.44-80.33-85.06-11.85-78.88 16-127.94 55.71-131.1 36.14-2.87 68.71 60.14 77.53 118.75zM223.65 409.53c3.13 33.28-14.86 64.34-42 69.66-27.4 5.36-58.71-16.37-65.09-49.19s17.75-34.56 47.32-40.21 55.99-20.4 59.77 19.74zM312 150.83c-8.81 58.62 7.33 90.67 52.9 97.41 50.66 7.49 71.52-26.44 80.33-85.06 11.86-78.89-16-128.22-55.7-131.1-36.4-2.64-68.71 60.13-77.53 118.75zM288.35 313.53c-3.13 33.27 14.86 64.34 42 69.66 27.4 5.36 58.71-16.37 65.09-49.19s-17.75-34.56-47.32-40.22-55.99-20.4-59.77 19.75z"
              />
            </svg>
            <p className="font-bold text-white">{steps}</p>
          </div>
          <div>
            <p className="text-white text-sm">Pasos</p>
          </div>
        </div>
        <div className="flex justify-center items-center flex-col">
          <div className="flex gap-2 mb-1">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1}
              stroke="white"
              className="w-5 h-5"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M15.362 5.214A8.252 8.252 0 0112 21 8.25 8.25 0 016.038 7.048 8.287 8.287 0 009 9.6a8.983 8.983 0 013.361-6.867 8.21 8.21 0 003 2.48z"
              />
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M12 18a3.75 3.75 0 00.495-7.467 5.99 5.99 0 00-1.925 3.546 5.974 5.974 0 01-2.133-1A3.75 3.75 0 0012 18z"
              />
            </svg>
            <p className="font-bold text-white">{calories}</p>
          </div>
          <div className="text-white text-sm">kCal</div>
        </div>
      </div>
    </div>
  );
}
