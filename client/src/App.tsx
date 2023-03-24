import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import Audio from "./Audio";

var Buffer = window.Buffer;

function App() {
  const [count, setCount] = useState(0);
  const [buffer, setBuffer] = useState<Buffer>();

  const ws = () => {
    const connection = new WebSocket("ws://localhost:8080/ws/stream");
    connection.onmessage = (event) => {
      console.log(event);
      console.log(Buffer);

      const newDataBuffer = Buffer.from(event.data);
      const inferBuffer = buffer ? buffer :Buffer.from([]) 
      const _buffer = Buffer.concat([inferBuffer, newDataBuffer]);
      setBuffer(_buffer);
    };

    connection.onopen = (event) => {
      const msg = JSON.stringify({
        spec: "stream",
        name: "yume",
      });

      connection.send(msg);
    };

    return connection;
  };
  useEffect(() => {
    const connection = ws();
  });

  return (
    <div></div>
    // <Audio></Audio>
  );
}

export default App;
