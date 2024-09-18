import { PGliteWorker } from '@electric-sql/pglite/worker'
import MainWorker from "./worker.ts?worker"
import { drizzle } from 'drizzle-orm/pglite'
import { PGlite } from '@electric-sql/pglite'

const client = new PGliteWorker(new MainWorker())
// @ts-ignore
export const db = drizzle(client as PGlite);
export * from "retrodb";
