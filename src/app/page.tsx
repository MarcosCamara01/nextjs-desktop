import UserInfo from "@/components/UserInfo";
import Greet from "@/components/greet";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Greet />
      <UserInfo />
    </main>
  );
}
