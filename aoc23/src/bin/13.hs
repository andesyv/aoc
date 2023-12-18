{- stack
  script
  --resolver lts-21.21
  --package split
-}

import Data.List
import Data.List.Split
import Data.Maybe
-- import Debug.Trace
import System.IO

data Direction = Horizontal | Vertical
  deriving (Eq)

areListsEqualToSameLength :: String -> String -> Bool
areListsEqualToSameLength a b = all (uncurry (==)) (zip a b)

isLineMirroredAroundPos :: Int -> String -> Bool
isLineMirroredAroundPos pos line = areListsEqualToSameLength (reverse (take pos line)) (drop pos line)

getMirrorOffset' :: Int -> [String] -> Maybe Int
getMirrorOffset' i pattern
  | length (head pattern) <= i = Nothing
  | all (isLineMirroredAroundPos i) pattern = Just i
  | otherwise = getMirrorOffset' (i + 1) pattern

getMirrorOffset :: [String] -> Maybe Int
getMirrorOffset = getMirrorOffset' 1

findOriginalMirrorPos :: [String] -> Maybe (Int, Direction)
findOriginalMirrorPos pattern = case getMirrorOffset pattern of
  Just i -> Just (i, Vertical)
  Nothing -> case getMirrorOffset (transpose pattern) of
    Just i -> Just (i, Horizontal)
    Nothing -> Nothing

getPatternValue :: [String] -> Int
getPatternValue pattern = case getMirrorOffset pattern of
  Just i -> i
  Nothing -> case getMirrorOffset (transpose pattern) of
    Just i -> 100 * i
    Nothing -> 0

getListDiffCount :: String -> String -> Int
getListDiffCount a b = foldr (\(x, y) acc -> acc + if x == y then 0 else 1) 0 (zip a b)

getLineDiffCount :: Int -> String -> Int
getLineDiffCount pos line = getListDiffCount (reverse (take pos line)) (drop pos line)

findFirstSwappablePos' :: Int -> [String] -> Maybe Int
findFirstSwappablePos' pos pattern
  | length (head pattern) <= pos || diffCount == 0 = Nothing
  | diffCount == 1 = Just pos
  | otherwise = findFirstSwappablePos' (1 + pos) pattern
  where
    diffCount = (sum . map (getLineDiffCount pos)) pattern

findFirstSwappablePos = findFirstSwappablePos' 1

findSwappablePositions' :: Int -> [String] -> [Int]
findSwappablePositions' pos pattern
  | length (head pattern) <= pos = []
  | diffCount == 1 = pos : findSwappablePositions' (1 + pos) pattern
  | otherwise = findSwappablePositions' (1 + pos) pattern
  where
    diffCount = (sum . map (getLineDiffCount pos)) pattern

findSwappablePositions = findSwappablePositions' 1

findHorizontalAndVerticalSwappablePositions :: [String] -> [(Int, Direction)]
findHorizontalAndVerticalSwappablePositions pattern = sortBy (\(a, _) (b, _) -> compare a b) (verticals ++ horizontals)
  where
    verticals = map (\x -> (x, Vertical)) $ findSwappablePositions pattern
    horizontals = map (\x -> (x, Horizontal)) $ findSwappablePositions (transpose pattern)

-- makeLineMirrored :: Int -> String -> String
-- makeLineMirrored pos line
--   | pos + pos < length line = (reverse . take pos . drop pos) line ++ drop pos line
--   | otherwise = reverse (makeLineMirrored (length line - pos) (reverse line))

calculateFlippedMirroredPatternValue :: [String] -> Int
calculateFlippedMirroredPatternValue pattern =
  if not (null mirrorCandidates)
    then case head mirrorCandidates of
      (pos, Vertical) -> pos
      (pos, Horizontal) -> pos * 100
    else 0
  where
    filterFunc pos = case findOriginalMirrorPos pattern of
      Just invalidPos -> pos /= invalidPos
      Nothing -> True
    mirrorCandidates = filter filterFunc (findHorizontalAndVerticalSwappablePositions pattern)

parse :: String -> [[String]]
parse = map lines . splitOn "\n\n"

getSumOfPatterns :: String -> Int
getSumOfPatterns = sum . map getPatternValue . parse

getSumOfOtherMirroredPatterns :: String -> Int
getSumOfOtherMirroredPatterns = sum . map calculateFlippedMirroredPatternValue . parse

-- Quite ugly multiline string literal, Haskell. :/
exampleInput =
  "#.##..##.\n\
  \..#.##.#.\n\
  \##......#\n\
  \##......#\n\
  \..#.##.#.\n\
  \..##..##.\n\
  \#.#.##.#.\n\
  \\n\
  \#...##..#\n\
  \#....#..#\n\
  \..##..###\n\
  \#####.##.\n\
  \#####.##.\n\
  \..##..###\n\
  \#....#..#"

exampleInput2 =
  "#.##.#...........\n\
  \#.##.##.......#..\n\
  \#.##.#####....###\n\
  \......#.#..##..#.\n\
  \.####..#.#....#.#\n\
  \#.##.####.####.##\n\
  \#....###.######.#\n\
  \.........##..##..\n\
  \.#..#.##........#"

-- main = print $ getSumOfOtherMirroredPatterns exampleInput2

main = do
  input <- readFile "src/inputs/13.txt"
  putStrLn ("Sum of original patterns: " ++ show (getSumOfPatterns input))
  putStrLn ("Sum of altered patterns: " ++ show (getSumOfOtherMirroredPatterns input))