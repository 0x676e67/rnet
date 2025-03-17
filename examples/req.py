import asyncio
import rnet
from rnet import Method


async def main():
    resp: rnet.Response = await rnet.request(Method.GET, "https://www.google.com/")
    print("Status Code: ", resp.status_code)
    print("Version: ", resp.version)
    print("Response URL: ", resp.url)
    print("Headers: ", resp.headers)
    print("Cookies: ", resp.cookies)
    print("Content-Length: ", resp.content_length)
    print("Encoding: ", resp.encoding)
    print("Remote Address: ", resp.remote_addr)
    print("Headers set-cookie: ", resp.headers["set-cookie"])

    for key in resp.headers:
        print(key)

    for key, value in resp.headers.items():
        print(f"{key}: {value}")
        
    for cookie in resp.cookies:
        print(cookie)
    # Close the response connection
    # resp.close()

    # text_content = await resp.text()
    # print("Text: ", text_content)

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


if __name__ == "__main__":
    asyncio.run(main())
