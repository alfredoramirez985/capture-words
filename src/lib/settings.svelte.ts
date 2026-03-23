export interface Session {
  id: number;
  name: string;
  description: string | null;
  is_favorite?: boolean;
  created_at: string;
}

interface Settings {
  captureDelaySeconds: number;
  activeSession: Session | null;
  geminiToken: string;
}

export const appSettings: Settings = $state({
  captureDelaySeconds: 10,
  activeSession: null,
  geminiToken: ""
});
