package adventofcode.y2022

import java.util.Scanner

private object Res

def readResourceAsString(resourceName: String) =
  val r = new Scanner(Res.getClass.getResourceAsStream(resourceName), "UTF-8")
  try
    r.useDelimiter("\\A").next()
  finally
    r.close()
