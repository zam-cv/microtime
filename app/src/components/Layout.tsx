export default function Layout({ children }: { children?: React.ReactNode }) {
  return (
    <div className="bg-gradient-to-b from-blue-500 to-purple-500">
      {children}
    </div>
  );
}