{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE GeneralizedNewtypeDeriving #-}
{-# LANGUAGE InstanceSigs #-}
{-# LANGUAGE EmptyDataDecls #-}
{-# LANGUAGE PolyKinds #-}

module BasicType.Phantom where

import Data.Data (Proxy (Proxy))

-- unit 就是 Phantom 类型
-- Phantom 为类型添加了额外的信息，从类型层面上保证数据安全
-- 导出 Num 和 Fractional 需要打开 GeneralizedNewtypeDeriving 扩展
newtype Temp unit = Temp Double
  deriving(Num, Fractional)

-- empty declarations
-- 需要打开 EmptyDataDecls 扩展
data F
data C

paperBurning :: Temp F
paperBurning = 451

absoluteZero :: Temp C
absoluteZero = -273.15

f2c :: Temp F -> Temp C
f2c (Temp f) = Temp((f - 32) * 5 / 9)


-- TYPE ERROR: Couldn't match type 'C' with 'F'
diff :: Temp C
diff = f2c paperBurning - absoluteZero

-- Proxy: 某个类型有一个值，但是这个值存在的唯一目的就是引用(refer to) 这个类型，
-- 值本身不会被用到，这样的类型和值被称为 proxies.
-- data Proxy t = Proxy, 其中参数 t 是 Phantom 类型
-- We may want to use proxies in situations when something we do depends on the 
-- type but not on a value
class UnitName u where
  unitName :: Proxy u -> String

-- haskell 中不能对类型进行匹配，想要区分相同函数中的不同类型只能通过定义 instance
-- 需要打开 InstanceSigs 扩展
instance UnitName C where
  unitName :: Proxy C -> String 
  unitName _ = "C"

instance UnitName F where
  unitName :: Proxy F -> String 
  unitName _ = "F"

-- 需要打开 PolyKinds 扩展，因为类型类 Class UnitName u 中，type u 的 Kind 是 Type,
-- 而 Temp 的 Kind 是 Type -> Type
instance UnitName Temp where
  unitName _ = "_unspecified unit_"

-- 需要打开 ScopedTypeVariables 扩展
instance UnitName unit => UnitName (Temp unit) where
  unitName _ = unitName (Proxy  :: Proxy unit)

instance UnitName unit => Show (Temp unit) where
  show (Temp t) = show t ++ "°" ++ unitName (Proxy :: Proxy unit)

unit :: forall u. UnitName u => Temp u -> String
unit _ = unitName (Proxy :: Proxy u)
