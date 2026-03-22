export interface Session {
  id: number;
  name: string;
  description: string | null;
}

interface Settings {
  captureDelaySeconds: number;
  activeSession: Session | null;
}

export const appSettings: Settings = $state({
  captureDelaySeconds: 10,
  activeSession: null
});
