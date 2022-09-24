{-# LANGUAGE TypeFamilies #-}
module TypeFamily.DataFamily where 

-- Data families provide a unified interface to different data representations

data family XList a

-- XListUnit 和 XBits 是数据构造器，分别创建了 XList () 和 XList Bool 的值
-- XList 是一个容器，存储不同的内容可以使用不同的存储方法
-- 如果存储的是 ()，使用一个整数来保存包含元素的个数
-- 如果存储的是 Bool，使用一个整数来表示 bits 的集合，另一个整数来表示元素的个数
newtype instance XList () = XListUnit Integer
data instance XList Bool = XBits Integer Integer

-- 通过类型类定义统一的接口
class XListable a where
  xempty   :: XList a
  xcons    :: a -> XList a -> XList a
  xheadMay :: XList a -> Maybe a

instance XListable () where
  xempty = XListUnit 0

  xcons () (XListUnit n) = XListUnit (n + 1)

  xheadMay (XListUnit 0) = Nothing
  xheadMay _ = Just ()

instance XListable Bool where
  xempty = XBits 0 0
  xcons b (XBits bits n) = XBits (bits * 2 + if b then 1 else 0) (n + 1)
  xheadMay (XBits bits n)
    | n <= 0 = Nothing
    | otherwise = Just (bits `mod` 2 /= 0)

testXList :: (Eq a, XListable a) => a => Bool
testXList a = xheadMay (xcons a xempty) == Just a
