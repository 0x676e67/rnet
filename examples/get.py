import asyncio
import rnet
from rnet import Impersonate


async def main():
    resp = await rnet.get(
        "https://tls.peet.ws/api/all",
        timeout=10,
    )
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers.to_dict())
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)

    text_content = await resp.text()
    print("Text: ", text_content)

    # text_with_charset = await resp.text_with_charset(encoding="utf-8")
    # print("Text with charset: ", text_with_charset)

    # bytes_content = await resp.bytes()
    # print("Bytes: ", bytes_content)
    # print("Bytes Array: ", list(bytes_content))

    # json_value =  await resp.json()
    # print("JSON: ", json_value)

    # json_value =  await resp.json_str()
    # print("JSON String: ", json_value)

    # json_value =  await resp.json_str_pretty()
    # print("JSON String Pretty: ", json_value)

    # Close the response connection
    # resp.close()


if __name__ == "__main__":
    asyncio.run(main())
