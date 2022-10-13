import axios from "axios";
import { decode } from "@msgpack/msgpack";

async function getMsgPack<T>(url: string): Promise<T | undefined> {
  let res = await axios
    .get<ArrayLike<number>>(url, { responseType: "arraybuffer" })
    .catch(console.error);
  if (res === undefined) return undefined;
  return decode(res.data) as T;
}
export default getMsgPack;
