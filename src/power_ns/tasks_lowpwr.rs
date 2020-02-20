#[doc = "Writer for register TASKS_LOWPWR"]
pub type W = crate::W<u32, super::TASKS_LOWPWR>;
#[doc = "Register TASKS_LOWPWR `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_LOWPWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable low power mode (variable latency)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_LOWPWR_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_LOWPWR_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_LOWPWR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_LOWPWR`"]
pub struct TASKS_LOWPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_LOWPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_LOWPWR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_LOWPWR_AW::TRIGGER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable low power mode (variable latency)"]
    #[inline(always)]
    pub fn tasks_lowpwr(&mut self) -> TASKS_LOWPWR_W {
        TASKS_LOWPWR_W { w: self }
    }
}
