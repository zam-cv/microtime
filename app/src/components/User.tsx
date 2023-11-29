export default function User({ name, children }: { name: string; children?: React.ReactNode }) {
  return (
    <div className="col-span-3 grid grid-cols-[1fr_1fr]">
      <div className="flex">
        <div className="flex items-center justify-center mr-1">
          <div className="p-2 flex items-center justify-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={2.5}
              stroke="currentColor"
              className="w-6 h-6 text-white"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
              />
            </svg>
          </div>
        </div>
        <div className="flex items-center">
          <h1 className="font-bold text-white font-sans text-lg">{name}</h1>
        </div>
      </div>
      <div>{children}</div>
    </div>
  );
}
