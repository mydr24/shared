// MyDR24 UI Components Library - shadcn-inspired design system
// Comprehensive healthcare-focused component library with accessibility and compliance features

pub mod button;
pub mod card;
pub mod input;
pub mod badge;
pub mod alert;
pub mod misc;

// Re-export all components for easy usage
pub use button::*;
pub use card::*;
pub use input::*;
pub use badge::*;
pub use alert::*;
pub use misc::*;

// Design system configuration
pub struct DesignSystem {
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub borders: Borders,
    pub shadows: Shadows,
    pub animations: Animations,
}

pub struct ColorPalette {
    // Primary colors for healthcare branding
    pub primary: &'static str,
    pub primary_foreground: &'static str,
    
    // Secondary colors
    pub secondary: &'static str,
    pub secondary_foreground: &'static str,
    
    // Destructive/error colors
    pub destructive: &'static str,
    pub destructive_foreground: &'static str,
    
    // Muted colors
    pub muted: &'static str,
    pub muted_foreground: &'static str,
    
    // Accent colors
    pub accent: &'static str,
    pub accent_foreground: &'static str,
    
    // Background colors
    pub background: &'static str,
    pub foreground: &'static str,
    
    // Card colors
    pub card: &'static str,
    pub card_foreground: &'static str,
    
    // Border colors
    pub border: &'static str,
    pub input: &'static str,
    
    // Ring colors
    pub ring: &'static str,
    
    // Chart colors for healthcare analytics
    pub chart_1: &'static str,
    pub chart_2: &'static str,
    pub chart_3: &'static str,
    pub chart_4: &'static str,
    pub chart_5: &'static str,
    
    // Healthcare-specific status colors
    pub emergency: &'static str,
    pub urgent: &'static str,
    pub normal: &'static str,
    pub low_priority: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
}

pub struct Typography {
    pub font_sans: &'static str,
    pub font_mono: &'static str,
}

pub struct Spacing {
    pub unit: &'static str, // Base spacing unit (typically 0.25rem)
}

pub struct Borders {
    pub radius: &'static str,
}

pub struct Shadows {
    pub sm: &'static str,
    pub default: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

pub struct Animations {
    pub duration_fast: &'static str,
    pub duration_normal: &'static str,
    pub duration_slow: &'static str,
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self {
            colors: ColorPalette {
                // Healthcare-focused primary colors (medical blue)
                primary: "hsl(210 100% 50%)",
                primary_foreground: "hsl(0 0% 98%)",
                
                secondary: "hsl(210 40% 96%)",
                secondary_foreground: "hsl(222.2 84% 4.9%)",
                
                destructive: "hsl(0 84.2% 60.2%)",
                destructive_foreground: "hsl(210 40% 98%)",
                
                muted: "hsl(210 40% 96%)",
                muted_foreground: "hsl(215.4 16.3% 46.9%)",
                
                accent: "hsl(210 40% 96%)",
                accent_foreground: "hsl(222.2 84% 4.9%)",
                
                background: "hsl(0 0% 100%)",
                foreground: "hsl(222.2 84% 4.9%)",
                
                card: "hsl(0 0% 100%)",
                card_foreground: "hsl(222.2 84% 4.9%)",
                
                border: "hsl(214.3 31.8% 91.4%)",
                input: "hsl(214.3 31.8% 91.4%)",
                
                ring: "hsl(210 100% 50%)",
                
                // Healthcare analytics colors
                chart_1: "hsl(210 100% 50%)",
                chart_2: "hsl(160 60% 45%)",
                chart_3: "hsl(30 95% 55%)",
                chart_4: "hsl(280 65% 60%)",
                chart_5: "hsl(340 75% 55%)",
                
                // Healthcare status colors
                emergency: "hsl(0 84% 60%)",      // Red
                urgent: "hsl(25 95% 55%)",        // Orange
                normal: "hsl(210 100% 50%)",      // Blue
                low_priority: "hsl(160 60% 45%)", // Green
                success: "hsl(142 76% 36%)",      // Green
                warning: "hsl(48 96% 53%)",       // Yellow
                info: "hsl(210 100% 50%)",        // Blue
            },
            typography: Typography {
                font_sans: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica Neue', Arial, sans-serif",
                font_mono: "'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace",
            },
            spacing: Spacing {
                unit: "0.25rem",
            },
            borders: Borders {
                radius: "0.5rem",
            },
            shadows: Shadows {
                sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)",
                default: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
                md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
                lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
                xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            },
            animations: Animations {
                duration_fast: "150ms",
                duration_normal: "200ms",
                duration_slow: "300ms",
            },
        }
    }
}

// Utility function to combine CSS classes with variant support
pub fn cn(classes: &[&str]) -> String {
    classes.join(" ")
}

// Size variants commonly used across components
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Sm,
    Default,
    Lg,
    Xl,
}

impl Size {
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Sm => "sm",
            Size::Default => "default",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }
}

// Variant types for consistent theming
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Variant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Variant::Default => "default",
            Variant::Destructive => "destructive",
            Variant::Outline => "outline",
            Variant::Secondary => "secondary",
            Variant::Ghost => "ghost",
            Variant::Link => "link",
        }
    }
}

// Healthcare-specific priority levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Emergency,
    Urgent,
    Normal,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Emergency => "emergency",
            Priority::Urgent => "urgent",
            Priority::Normal => "normal",
            Priority::Low => "low",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            Priority::Emergency => "hsl(0 84% 60%)",
            Priority::Urgent => "hsl(25 95% 55%)",
            Priority::Normal => "hsl(210 100% 50%)",
            Priority::Low => "hsl(160 60% 45%)",
        }
    }
}

// Healthcare status types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthcareStatus {
    Active,
    Inactive,
    Pending,
    Verified,
    Suspended,
    Emergency,
    // Additional variants needed by apps
    Stable,
    NeedsAttention,
    Critical,
}

impl HealthcareStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthcareStatus::Active => "active",
            HealthcareStatus::Inactive => "inactive",
            HealthcareStatus::Pending => "pending",
            HealthcareStatus::Verified => "verified",
            HealthcareStatus::Suspended => "suspended",
            HealthcareStatus::Emergency => "emergency",
            HealthcareStatus::Stable => "stable",
            HealthcareStatus::NeedsAttention => "needs-attention",
            HealthcareStatus::Critical => "critical",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            HealthcareStatus::Active => "hsl(142 76% 36%)",
            HealthcareStatus::Inactive => "hsl(0 0% 50%)",
            HealthcareStatus::Pending => "hsl(48 96% 53%)",
            HealthcareStatus::Verified => "hsl(210 100% 50%)",
            HealthcareStatus::Suspended => "hsl(0 84% 60%)",
            HealthcareStatus::Emergency => "hsl(0 84% 60%)",
            HealthcareStatus::Stable => "hsl(142 76% 36%)",
            HealthcareStatus::NeedsAttention => "hsl(48 96% 53%)",
            HealthcareStatus::Critical => "hsl(0 84% 60%)",
        }
    }
}

// Statistics trend enum for healthcare metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatsTrend {
    Up,
    Down,
    Stable,
}

impl StatsTrend {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatsTrend::Up => "up",
            StatsTrend::Down => "down", 
            StatsTrend::Stable => "stable",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            StatsTrend::Up => "↗️",
            StatsTrend::Down => "↘️",
            StatsTrend::Stable => "➡️",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            StatsTrend::Up => "hsl(142 76% 36%)",
            StatsTrend::Down => "hsl(0 84% 60%)",
            StatsTrend::Stable => "hsl(0 0% 50%)",
        }
    }
}

// Card variant types for different healthcare contexts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardVariant {
    Default,
    Provider,
    Patient,
    Organization,
    Emergency,
    Stats,
}

impl CardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardVariant::Default => "default",
            CardVariant::Provider => "provider",
            CardVariant::Patient => "patient",
            CardVariant::Organization => "organization",
            CardVariant::Emergency => "emergency",
            CardVariant::Stats => "stats",
        }
    }
}
