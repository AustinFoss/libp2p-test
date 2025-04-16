import type { AstroIntegration, ContainerRenderer } from 'astro';
import { type Options as ViteSolidPluginOptions } from 'vite-plugin-solid';
export declare function getContainerRenderer(): ContainerRenderer;
export interface Options extends Pick<ViteSolidPluginOptions, 'include' | 'exclude'> {
    devtools?: boolean;
}
export default function (options?: Options): AstroIntegration;
