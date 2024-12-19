import { writable } from "svelte/store";
import type { CameraPreset } from "../types/cameraPreset";

export const cameraPresets = writable<CameraPreset[]>([]);