interface Config {
  apiUrl: string;
}

export default function getConfig(): Config {
  return {
    apiUrl: "http://localhost:8000",
  };
}
