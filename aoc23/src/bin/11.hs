{- stack
  script
  --resolver lts-21.21
  --package split
-}

-- Weirdly enough, ^this stack script hint requires at least one package in the current version of Stack. Otherwise it just freezes.
-- Ideally I think a proper haskell project setup would be better, but neither Stack or Cabal are very user friendly for new users...

import Data.List
import Debug.Trace

data Direction = Vertical | Horizontal
  deriving (Show)

expandEmptySpaceLines :: [String] -> [String]
expandEmptySpaceLines [] = []
expandEmptySpaceLines (x : xs)
  | all (== '.') x = x : (x : expandEmptySpaceLines xs)
  | otherwise = x : expandEmptySpaceLines xs

expandEmptySpace :: [String] -> [String]
expandEmptySpace = transpose . expandEmptySpaceLines . transpose . expandEmptySpaceLines

getSpaceDivisionLinePositions :: [String] -> [(Int, Direction)]
getSpaceDivisionLinePositions galaxyMap = map (,Horizontal) horizontals ++ map (,Vertical) verticals
  where
    horizontals = [i | (line, i) <- zip galaxyMap [0 ..], all (== '.') line]
    verticals = [i | (line, i) <- zip (transpose galaxyMap) [0 ..], all (== '.') line]

findGalaxyPositions :: [String] -> [(Int, Int)]
findGalaxyPositions galaxyMap = map (\(_, x, y) -> (x, y)) . filter (\(c, _, _) -> c == '#') . concat $ [[(c, x, y) | (c, x) <- zip line [0 ..]] | (line, y) <- zip galaxyMap [0 ..]]

getDistanceBetweenGalaxies :: (Int, Int) -> (Int, Int) -> Int
getDistanceBetweenGalaxies (x1, y1) (x2, y2) = dx + dy
  where
    dx = abs (x2 - x1)
    dy = abs (y2 - y1)

isSpaceDivisionApplicable :: (Int, Int) -> (Int, Int) -> (Int, Direction) -> Bool
isSpaceDivisionApplicable (x1, y1) (x2, y2) (divPos, dir) = case dir of
  Vertical -> x1 < divPos && divPos < x2 || x2 < divPos && divPos < x1
  Horizontal -> y1 < divPos && divPos < y2 || y2 < divPos && divPos < y1

getDistanceBetweenGalaxiesWithSpaceDivisions :: [(Int, Direction)] -> (Int, Int) -> (Int, Int) -> Int
getDistanceBetweenGalaxiesWithSpaceDivisions spaceDivisions (x1, y1) (x2, y2) = dx + dy + additionalSpace
  where
    dx = abs (x2 - x1)
    dy = abs (y2 - y1)
    applicableSpaceDivisions = filter (isSpaceDivisionApplicable (x1, y1) (x2, y2)) spaceDivisions
    additionalSpace = (1000000 - 1) * length applicableSpaceDivisions
    -- debugMsg = "(" ++ show x1 ++ ", " ++ show y1 ++ ") -> ("  ++ show x2 ++ ", " ++ show y2 ++ ") = (" ++ show dx ++ ", " ++ show dy ++ "), additional spaces: " ++ show applicableSpaceDivisions ++ ", distance: " ++ show (dx + dy + additionalSpace)

parse = lines

sortTuplesToSet a b
  | fst a < fst b = (a, b)
  | fst a == fst b && snd a < snd b = (a, b)
  | otherwise = (b, a)

getSumOfDistancesBetweenGalaxies :: String -> Int
getSumOfDistancesBetweenGalaxies input =
  foldr (\(pos1, pos2) acc -> acc + getDistanceBetweenGalaxies pos1 pos2) 0 $
    nub [sortTuplesToSet pos1 pos2 | pos1 <- galaxyPositions, pos2 <- galaxyPositions]
  where
    galaxyPositions = findGalaxyPositions $ expandEmptySpace $ parse input

-- Note to self: I really should've just increased the galaxy positions with the space divisions as that would've reduced the runtime by A LOT
getSumOfDistancesBetweenGalaxiesWithLargeDistances :: String -> Int
getSumOfDistancesBetweenGalaxiesWithLargeDistances input =
  foldr (\(pos1, pos2) acc -> acc + getDistanceBetweenGalaxiesWithSpaceDivisions spaceDivisions pos1 pos2) 0 $
    nub [sortTuplesToSet pos1 pos2 | pos1 <- galaxyPositions, pos2 <- galaxyPositions]
  where
    galaxyMap = parse input
    galaxyPositions = findGalaxyPositions galaxyMap
    spaceDivisions = getSpaceDivisionLinePositions galaxyMap

exampleInput =
  "...#......\n\
  \.......#..\n\
  \#.........\n\
  \..........\n\
  \......#...\n\
  \.#........\n\
  \.........#\n\
  \..........\n\
  \.......#..\n\
  \#...#....."

-- main = print $ getDistanceBetweenGalaxies (findGalaxyPositions (parse exampleInput) !! 7) (findGalaxyPositions (parse exampleInput) !! 8)
-- main = print $ "start: " ++ show (findGalaxyPositions (parse exampleInput) !! 4) ++ ", end: " ++ show (findGalaxyPositions (parse exampleInput) !! 8) ++", Distance: " ++ show (getDistanceBetweenGalaxies (findGalaxyPositions (parse exampleInput) !! 4) (findGalaxyPositions (parse exampleInput) !! 8))
-- main = print $ "Sum of distances: " ++ show (getSumOfDistancesBetweenGalaxiesWithLargeDistances exampleInput)
-- main = print $ show $ getSpaceDivisionLinePositions $ parse exampleInput

main = do
  input <- readFile "src/inputs/11.txt"
  putStrLn $ "Sum of distances between galaxies: " ++ show (getSumOfDistancesBetweenGalaxies input)
  putStrLn $ "Sum of distances between galaxies with increased distances: " ++ show (getSumOfDistancesBetweenGalaxiesWithLargeDistances input)