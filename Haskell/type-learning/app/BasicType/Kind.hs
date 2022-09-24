{-# LANGUAGE DataKinds #-}
{-# LANGUAGE KindSignatures #-}
{-# LANGUAGE GeneralisedNewtypeDeriving #-}
{-# LANGUAGE AllowAmbiguousTypes #-}
{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE ScopedTypeVariables #-}
module BasicType.Kind where

data TempUnits = F | C

-- 需要打开 DataKinds 和 KindSignatures，打开 DataKinds 将
-- TempUnits 升格到 Kind 级别，F 和 C 升格到 type 级别，因此
-- 当前的 Kind 有 Type, Constant 和 TempUnits
newtype Temp (u :: TempUnits) = Temp Double
  deriving(Num, Fractional)

paperBurning :: Temp F
paperBurning = 451

absoluteZero :: Temp C
absoluteZero = 0

f2c :: Temp F -> Temp C
f2c (Temp f) = Temp ((f -32) * 5 / 9)

-- 通过指定 u 的 Kind 是 TempUnits，除了 F 和 C 之外，不允许再
-- 定义 UniteName 的实例
class UnitName (u :: TempUnits) where
  unitName :: String

instance UnitName C where
  unitName = "C"

instance UnitName F where
  unitName = "F"

instance UnitName u => Show (Temp u) where
  show (Temp t) = show t ++ "°" ++ unitName @u

unit :: forall u. UnitName u => Temp u -> String
unit _ = unitName @u
