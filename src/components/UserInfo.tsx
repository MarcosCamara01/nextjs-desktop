"use client";

import { invoke } from "@tauri-apps/api";
import { useState, useEffect } from "react";

const SystemInfo = () => {
  const [systemInfo, setSystemInfo] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const info = await invoke("get_system_info");
        setSystemInfo(info as string);
      } catch (error) {
        console.error("Error fetching system info:", error);
      }
    })();
  }, []);

  return (
    <div>
      <h1>System Information</h1>
      {systemInfo ? (
        <p className="max-w-[600px]">{systemInfo}</p>
      ) : (
        <p>Loading...</p>
      )}
    </div>
  );
};

export default SystemInfo;
