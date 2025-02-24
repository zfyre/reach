import sys
import asyncio
import argparse
from crawl4ai import *

sys.stdout.reconfigure(encoding='utf-8')

async def main(url):
    async with AsyncWebCrawler() as crawler:
        result = await crawler.arun(url=url)
        print(result.markdown)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Web crawler for academic papers')
    parser.add_argument('--url', type=str, help='URL to crawl')
    args = parser.parse_args()
    
    asyncio.run(main(args.url))