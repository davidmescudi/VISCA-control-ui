import { writable } from "svelte/store";
import type { Camera } from "../types/camera";

export const cameras = writable<Camera[]>([]);