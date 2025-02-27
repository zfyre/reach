from cx_Freeze import setup, Executable 
  
setup(name = "Crawler" , 
      version = "0.1" , 
      description = "Uses Crawl4Ai to crawl the provided URL" , 
      executables = [Executable("src/scripts/crawl.py")]) 